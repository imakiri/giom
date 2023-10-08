struct Coordinates {
    /// from left to right direction
    pub x: f64,
    /// from top to bottom direction
    pub y: f64,
}

struct Link<Target, Link> {
    /// unique id
    pub id: Uuid,
    /// displayed name
    pub name: String,
    /// a target is to which this link belongs
    pub target: Target,
    /// slice of urls to web resources with information
    pub link: Link,
}

struct Category<Image> {
    /// unique id
    pub id: Uuid,
    /// displayed name
    pub name: String,
    /// description
    pub description: String,
    /// displayed icon in webp format
    pub icon: Image,
}

type CategoryLink<Image, Link> = Link<Category<Image>, Link>;

struct Group<Image> {
    /// unique id
    pub id: Uuid,
    /// displayed name
    pub name: String,
    /// description
    pub description: String,
    /// displayed icon in webp format
    pub icon: Image,
}

type GroupLink<Image, Link> = Link<Group<Image>, Link>;


struct Map<Image> {
    /// unique id
    pub id: Uuid,
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

struct Sight<Category> {
    /// unique id
    pub id: Uuid,
    /// Category
    pub category: Category,
}

enum DependenceKind {
    NotAccessibleBeforeInteraction = 1
}

struct Dependence<Sight, Kind> {
    /// unique id
    pub id: Uuid,
    /// Sight
    pub sight: Sight,
    /// that depends on other Sight
    pub depends_on: Sight,
    /// kind of dependence
    pub kind: Kind
}

struct Location<Map, Sight> {
    /// unique id
    pub id: Uuid,
    /// Map to which this location belongs
    pub map: Map,
    /// Sight to which this location belongs
    pub sight: Sight,
    /// position on the Map
    pub position: Coordinates,
}

struct Observation<Image, Location> {
    /// unique id
    pub id: Uuid,
    /// Location to which this Observation belongs
    pub location: Location,
    /// image of an observation in webp format
    pub image: Image,
}

use uuid::Uuid;
use url::Url;
use webp::WebPImage;
use std::collections::HashMap;









