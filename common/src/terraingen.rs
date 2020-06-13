use crate::world::Material;

pub struct VoronoiInfo {
    pub location: na::Vector3<f64>,
    pub material: Material,
}
impl VoronoiInfo {
    pub fn new(mat: Material, elev: f64, rain: f64, temp: f64) -> VoronoiInfo {
        VoronoiInfo {
            location: na::Vector3::new(elev, rain, temp),
            material: mat,
        }
    }

    pub fn terraingen_voronoi(y: na::Vector3<f64>) -> Material {
        const NUM_CHOICES: usize = 16;
        let voronoi_choices: [VoronoiInfo; NUM_CHOICES] = [
            VoronoiInfo::new(Material::Stone, 0.0,-3.0, 0.0),
            VoronoiInfo::new(Material::Gravelstone, 0.0,-1.0, 0.0),
            VoronoiInfo::new(Material::Graveldirt, 0.0, 2.0, 0.0),
            VoronoiInfo::new(Material::Dirt, 0.0, 2.5, 0.0),
            VoronoiInfo::new(Material::Grass, 0.0, 3.5, 0.0),
            VoronoiInfo::new(Material::Flowergrass, 0.0, 4.75, 0.0),

            VoronoiInfo::new(Material::Greystone, 0.0, -3.5, -4.0),
            VoronoiInfo::new(Material::Redstone, 0.0, 2.5, 4.0),

            VoronoiInfo::new(Material::Blackstone, 0.0, -2.0, 4.0),
            VoronoiInfo::new(Material::GreySand, 0.0, 0.0, 4.0),
            VoronoiInfo::new(Material::Sand, 0.0, 2.0, 5.0),
            VoronoiInfo::new(Material::Redsand, 0.0, 2.5, 4.0),
            VoronoiInfo::new(Material::Mud, 0.0, 3.75, 5.0),

            VoronoiInfo::new(Material::Lava, 0.0, -2.0, 10.0),

            VoronoiInfo::new(Material::Ice, 0.0, 5.0, -6.0),
            VoronoiInfo::new(Material::Snow, 1.0, 2.5, -5.0),
        ];

        let mut voxel_mat: Material;

        let mut dist = na::norm(&(&voronoi_choices[0].location - &y));
        voxel_mat = voronoi_choices[0].material;
        for i in 1..NUM_CHOICES {
            let d = na::norm(&(&voronoi_choices[i].location - &y));
            if d <= dist {
                dist = d;
                voxel_mat = voronoi_choices[i].material;
            };
        }

        voxel_mat
    }
}
