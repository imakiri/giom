pub struct Coordinates {
    /// from left to right direction
    pub x: f64,
    /// from top to bottom direction
    pub y: f64,
}

pub struct Link<Id, Target, URL> {
    /// unique id
    pub id: Id,
    /// displayed name
    pub name: String,
    /// a target is to which this link belongs
    pub target: Target,
    /// slice of urls to web resources with information
    pub link: URL,
}

pub struct Category<Id, Image> {
    /// unique id
    pub id: Id,
    /// displayed name
    pub name: String,
    /// description
    pub description: String,
    /// displayed icon in webp format
    pub icon: Image,
}

pub type CategoryLink<Id, Image, URL> = Link<Id, Category<Id, Image>, URL>;

pub struct Group<Id, Image> {
    /// unique id
    pub id: Id,
    /// displayed name
    pub name: String,
    /// description
    pub description: String,
    /// displayed icon in webp format
    pub icon: Image,
}

pub type GroupLink<Id, Image, URL> = Link<Id, Group<Id, Image>, URL>;

pub struct Map<Id, Image> {
    /// unique id
    pub id: Id,
    /// displayed name
    pub name: String,
    /// description
    pub description: String,
    /// image anchors:
    ///
    /// the first anchor has fixed position (0, 0) on the map,
    ///
    /// the second anchor has fixed position (1, 1) on the map
    ///
    /// the value of an anchor is coordinates of a corresponding map point(0,0 or 1,1) on the image
    pub anchors: [Coordinates; 2],
    /// image of a map in webp format
    pub image: Image,
}

pub struct Sight<Id, Category> {
    /// unique id
    pub id: Id,
    /// Category
    pub category: Category,
}

pub enum DependenceKind {
    NotAccessibleBeforeInteraction = 1
}

pub struct Dependence<Id, Sight, Kind> {
    /// unique id
    pub id: Id,
    /// Sight
    pub sight: Sight,
    /// that depends on other Sight
    pub depends_on: Sight,
    /// kind of dependence
    pub kind: Kind,
}

pub struct Location<Id, Map, Sight> {
    /// unique id
    pub id: Id,
    /// Map to which this location belongs
    pub map: Map,
    /// Sight to which this location belongs
    pub sight: Sight,
    /// position on the Map
    pub position: Coordinates,
}

pub struct Observation<Id, Image, Location> {
    /// unique id
    pub id: Id,
    /// Location to which this Observation belongs
    pub location: Location,
    /// image of an observation in webp format
    pub image: Image,
}












