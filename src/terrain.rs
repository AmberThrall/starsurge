use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};
use super::Position;

pub const CELL_SIZE: Vec3 = Vec3::ONE;

#[derive(Resource)]
pub struct TerrainMaterial(Handle<StandardMaterial>);

/// Terrain vertex data
#[derive(Default, Debug)]
pub struct TerrainVertex {
    pub altitude: f32,
    pub color: Color,
}

/// Terrain size in quads.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct TerrainSize {
    pub width: usize,
    pub height: usize,
}

impl TerrainSize {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }

    pub fn square(size: usize) -> Self {
        Self::new(size, size)
    }

    /// Number of quads the terrain has/will have.
    pub fn num_quads(&self) -> usize {
        self.width * self.height
    }

    /// Number of vertices the terrain has/will have.
    pub fn num_vertices(&self) -> usize {
        (self.width + 1) * (self.height + 1)
    }
}

/// Builds a terrain based off a heightmap and colormap images.
/// 
/// Creates and inserts a Terrain component to the owner. Both images must have the same size or a panic with incur.
#[derive(Component)]
pub struct TerrainBuilder {
    /// Heightmap image handle. Altitude is determined by the red-channel.
    pub heightmap: Handle<Image>,
    /// Colormap image handle. Designates the color of each vertex.
    pub colormap: Handle<Image>,
    /// Altitude scale. If the heightmap at a position is 255, the terrain will have altitude `max_altitude`.
    pub max_altitude: f32,
}

/// Terrain component.
#[derive(Component)]
pub struct Terrain {
    /// Number of quads in each direction.
    pub size: TerrainSize,
    /// Vertex data
    pub data: Vec<TerrainVertex>,
}

impl Terrain {
    /// Create a plane of a specificed size.
    pub fn plane(size: TerrainSize) -> Self {
        let mut data = Vec::new();
        for _ in 0..size.num_vertices() {
            data.push(TerrainVertex::default());
        }

        Self {
            size,
            data,
        }
    }

    /// Create a terrain generated by sin waves.
    pub fn sinisuidal(size: TerrainSize, amplitude: f32) -> Self {
        let mut t = Self::plane(size);
        for y in 0..(size.height+1) {
            for x in 0..(size.width+1) {
                if let Some(cell) = t.get_vertex_mut(x, y) {
                    let h = amplitude * ((y * (size.width + 1) + x) as f32).sin();
                    cell.altitude = h;
                    let grayscale = (y as f32) / (size.height as f32 + 1.0);
                    cell.color = Color::rgb(grayscale, grayscale, grayscale);
                }
            }
        }
        t
    }

    /// Create a terrain given a heightmap and colormap. See TerrainBuilder.
    pub fn from_images(heightmap: &Image, colormap: &Image, max_altitude: f32) -> Self {
        let img_size = (heightmap.size().x as usize, heightmap.size().y as usize);
        let size = TerrainSize::new(heightmap.size().x as usize - 1, heightmap.size().y as usize - 1);

        let mut data = Vec::new();
        for vy in 0..img_size.1 {
            for vx in 0..img_size.0 {
                let i = (vy * img_size.0 + vx) * 4;
                let y = *heightmap.data.get(i).unwrap();
                let r = *colormap.data.get(i).unwrap();
                let g = *colormap.data.get(i + 1).unwrap();
                let b = *colormap.data.get(i + 2).unwrap();
    
                data.push(TerrainVertex {
                    altitude: ((y as f32) / 255.0) * max_altitude,
                    color: Color::rgb_u8(r, g, b),
                });
            }
        }
        
        Self {
            size,
            data,
        }
    }

    /// Convert a grid position to a world position.
    /// 
    /// The y-coordinate is determined by taking the average of the quad's four vertices.
    pub fn get_world_position(&self, pos: &Position) -> Vec3 {
        let vx = (self.size.width / 2) as i32 + pos.x;
        let vy = (self.size.height / 2) as i32 + pos.y;
        let vert_pos = vec![
            self.get_vertex_position(vx as usize, vy as usize),
            self.get_vertex_position(vx as usize + 1, vy as usize),
            self.get_vertex_position(vx as usize + 1, vy as usize + 1),
            self.get_vertex_position(vx as usize, vy as usize + 1),
        ];
        let middle = vert_pos[0] + CELL_SIZE / 2.0;

        let avg_y = (vert_pos[0].y + vert_pos[1].y + vert_pos[2].y + vert_pos[3].y) / 4.0;

        Vec3::new(middle.x, avg_y, middle.z)
    }

    /// Get the world position (including y) of a specific vertex.
    pub fn get_vertex_position(&self, vx: usize, vy: usize) -> Vec3 {
        let half_width = (self.size.width as f32) / 2.0;
        let half_height = (self.size.height as f32) / 2.0;

        let terrain_bl = Vec3::new(-half_width, 0.0, -half_height) * CELL_SIZE;
        let pos = terrain_bl + Vec3::new(vx as f32, 0.0, vy as f32) * CELL_SIZE;
        let vert = self.get_vertex(vx, vy).unwrap();

        Vec3::new(pos.x, vert.altitude, pos.z)
    }

    /// Calculate a vertices normal vector.
    pub fn get_vertex_normal(&self, _vx: usize, _vy: usize) -> Vec3 {
        Vec3::Y
    }

    /// Get a reference to a terrain vertex
    pub fn get_vertex(&self, vx: usize, vy: usize) -> Option<&TerrainVertex> {
        self.data.get(vy * (self.size.width + 1) + vx)
    }

    /// Get a mutable reference to a terain vertex.
    pub fn get_vertex_mut(&mut self, vx: usize, vy: usize) -> Option<&mut TerrainVertex> {
        self.data.get_mut(vy * (self.size.width + 1) + vx)
    }

    /// Generate the terrain's mesh. Called by TerrainPlugin when a Terrain component is added to an entity.
    pub fn generate_mesh(&self) -> Mesh {
        let mut indices = Vec::new();
        let mut vertices = Vec::new();
        let mut normals = Vec::new();
        let mut colors = Vec::new();
    
        let verts_grid_size = (self.size.width + 1, self.size.height + 1);
        
        // Build vertex data
        /* Vertices for a 4x2 grid:
             +----+----+-----+----+
           10|  11|  12|   13|  14|
             |    |    |     |    |
             +----+----+-----+----+
            5|   6|   7|    8|   9|
             |    |    |     |    |
             +----+----+-----+----+
            0    1    2     3    4
        */
        for vy in 0..verts_grid_size.1 {
            for vx in 0..verts_grid_size.0 {
                let pos = self.get_vertex_position(vx, vy);
                let normal = self.get_vertex_normal(vx, vy);
                let vert = self.get_vertex(vx, vy).unwrap();
    
                vertices.push([pos.x, pos.y, pos.z]);
                normals.push([normal.x, normal.y, normal.z]);
                colors.push([vert.color.r(), vert.color.g(), vert.color.b(), 1.0]);
            }
        }
    
        // Build indices
        /* a -> c -> d && a -> d -> b
             +----+
            c|   d|
             |    |
             +----+
            a    b
         */
        for y in 0..self.size.height {
            for x in 0..self.size.width {
                let a = (y * (verts_grid_size.0) + x) as u32;
                let b = (y * (verts_grid_size.0) + (x + 1)) as u32;
                let c = ((y + 1) * (verts_grid_size.0) + x) as u32;
                let d = ((y + 1) * (verts_grid_size.0) + (x + 1)) as u32;
                indices.push(a); indices.push(c); indices.push(d);
                indices.push(a); indices.push(d); indices.push(b);
            }
        }
    
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
        //mesh.compute_flat_normals(); // panics if done after set_indices
        mesh.set_indices(Some(Indices::U32(indices)));
    
        mesh
    }
}

impl Default for Terrain {
    fn default() -> Self {
        Self::plane(TerrainSize::default())
    }
}

/// Terrain plugin handling loading and positions.
pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
       app.add_startup_system(setup_material)
            .add_system(build_terrains.before(generate_mesh))
            .add_system(generate_mesh);
    }
}

fn setup_material(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mat = materials.add(StandardMaterial { 
        base_color: Color::WHITE, 
        ..default()
    });
    commands.insert_resource(TerrainMaterial(mat));
}

fn build_terrains(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    query: Query<(Entity, &TerrainBuilder)>,
) {
    for (entity, builder) in query.iter() {
        if let Some(heightmap_img) = images.get(&builder.heightmap) {
            if let Some(colormap_img) = images.get(&builder.colormap) {
                // remove the TerrainBuilder and insert the terrain.
                commands.entity(entity)
                    .insert(Terrain::from_images(heightmap_img, colormap_img, builder.max_altitude))
                    .remove::<TerrainBuilder>();

                // We no longer need the images. unload them
                images.remove(builder.heightmap.clone());
                images.remove(builder.colormap.clone());
            }
        }
    }
}

fn generate_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    material: Res<TerrainMaterial>,
    query: Query<(Entity, &Terrain), Added<Terrain>>,
) {
    for (entity, terrain) in query.iter() {
        let mesh = meshes.add(terrain.generate_mesh());

        commands.entity(entity).insert(PbrBundle {
            mesh: mesh,
            material: material.0.clone_weak(),
            ..default()
        });
    }
}