create table if not exists categories
(
    id          BLOB not null
        constraint category_pk
            primary key,
    name        TEXT not null,
    description TEXT not null,
    links       TEXT not null,
    icon        BLOB not null
)
    strict;

create unique index if not exists category_id_uindex
    on categories (id desc);

create table if not exists category_links
(
    id          BLOB not null
        constraint category_links_pk
            primary key,
    name        TEXT not null,
    category_id BLOB not null
        constraint category_links_categories_id_fk
            references categories,
    link        TEXT not null
)
    strict;

create unique index if not exists category_links_id_uindex
    on category_links (id desc);

create index if not exists category_links_category_id_index
    on category_links (category_id desc);

create table if not exists groups
(
    id          BLOB not null
        constraint groups_pk
            primary key,
    name        TEXT not null,
    description integer,
    icon        BLOB not null
)
    strict;

create unique index if not exists groups_id_uindex
    on groups (id desc);

create table if not exists group_links
(
    id       BLOB not null
        constraint group_links_pk
            primary key,
    name     TEXT not null,
    group_id BLOB not null
        constraint group_links_groups_id_fk
            references groups,
    link     TEXT not null
)
    strict;

create unique index if not exists group_links_id_uindex
    on group_links (id desc);

create index if not exists group_links_group_id_uindex
    on group_links (group_id desc);


create table if not exists maps
(
    id          BLOB not null
        constraint maps_pk
            primary key,
    name        TEXT not null,
    description TEXT not null,
    anchor0     REAL not null,
    anchor1     REAL not null,
    image       BLOB not null
)
    strict;

create unique index if not exists maps_id_uindex
    on maps (id desc);

create table if not exists sights
(
    id          BLOB not null
        constraint sights_pk
            primary key,
    category_id BLOB not null
        constraint sights_categories_id_fk
            references categories
)
    strict;

create unique index if not exists sights_id_uindex
    on sights (id desc);

create index if not exists sights_category_id_uindex
    on sights (category_id desc);

create table if not exists sight_dependencies
(
    id                  BLOB    not null
        constraint sight_dependencies_pk
            primary key,
    sight_id            BLOB    not null
        constraint sight_dependencies_sights_id_fk
            references sights,
    depends_on_sight_id BLOB    not null
        constraint sight_dependencies_sights_id_fk2
            references sights,
    kind                integer not null
)
    strict;

create index if not exists sight_dependencies_sight_id_index
    on sight_dependencies (sight_id desc);

create index if not exists sight_dependencies_depends_on_sight_id_index
    on sight_dependencies (depends_on_sight_id desc);

create table if not exists sight_groups
(
    id                  BLOB    not null
        constraint sight_groups_pk
            primary key,
    sight_id            BLOB    not null
        constraint sight_groups_sights_id_fk
            references sights,
    belongs_to_group_id BLOB    not null
        constraint sight_groups_sights_id_fk2
            references sights,
    kind                integer not null
)
    strict;

create index if not exists sight_groups_sight_id_index
    on sight_groups (sight_id desc);

create index if not exists sight_groups_belongs_to_group_id_index
    on sight_groups (belongs_to_group_id desc);

create table if not exists locations
(
    id         BLOB not null
        constraint locations_pk
            primary key,
    map_id     BLOB not null
        constraint locations_maps_id_fk
            references maps,
    sight_id   BLOB not null
        constraint locations_sight_id_fk
            references sights,
    position_x REAL not null,
    position_y REAL not null
)
    strict;

create unique index if not exists locations_id_uindex
    on locations (id desc);

create index if not exists locations_map_id_uindex
    on locations (map_id desc);

create index if not exists locations_sight_id_uindex
    on locations (sight_id desc);

create table if not exists observations
(
    id          BLOB not null
        constraint observations_pk
            primary key,
    location_id BLOB not null
        constraint observations_locations_id_fk
            references locations,
    image       BLOB not null
)
    strict;

create unique index if not exists observations_id_uindex
    on observations (id desc);

create index if not exists observations_location_id_index
    on observations (location_id desc);
