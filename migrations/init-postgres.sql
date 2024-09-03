create type user_status as enum ('clean', 'temp_banned', 'perma_banned');
create type project_status as enum ('clean', 'dropped', 'watched');
create type group_user_status as enum ('mod', 'user', 'blacklist');
create table users (
  id varchar(255) unique not null,
  email varchar(255) unique not null,
  password varchar(512) not null,
  dob date not null,
  username varchar(255) unique not null,
  fullname varchar(255) not null,
  alias varchar(255),
  address varchar(255),
  org_name varchar(255),
  avatar_url varchar(255),
  status user_status default 'clean',
  created_at timestamp not null default current_timestamp,
  updated_at timestamp not null default current_timestamp,
  deleted_at timestamp default null,
  constraint pk_users primary key (id)
);

create table projects (
  id varchar(255) not null,
  mirror_links varchar(255) default null,
  owner_id varchar(255) not null,
  vir_fs_key varchar(255),
  status project_status default 'clean',
  created_at timestamp not null default current_timestamp,
  updated_at timestamp not null default current_timestamp,
  deleted_at timestamp default null,
  constraint pk_projects primary key (id),
  constraint fk_projects_users foreign key (owner_id)
  references users(id)
);

create table groups (
  id varchar(255) not null ,
  name varchar(255) not null,
  creator_id varchar(255) not null,
  project_id varchar(255) not null,
  created_at timestamp not null default current_timestamp,
  updated_at timestamp not null default current_timestamp,
  deleted_at timestamp default null,
  constraint pk_groups primary key (id),
  constraint fk_groups_users foreign key (creator_id)
  references users(id),
  constraint fk_groups_projects foreign key (project_id)
  references projects(id)
);

create table groups_users (
  id varchar(255) not null,
  user_id varchar(255) not null,
  joined_date timestamp not null default current_timestamp,
  status group_user_status default 'user',
  created_at timestamp not null default current_timestamp,
  updated_at timestamp not null default current_timestamp,
  deleted_at timestamp default null,
  constraint pk_groups_users primary key (id)
);

create table messages (
  id serial,
  sender_id varchar(255),
  receiver_id varchar(255),
  forwarded_from varchar(255) default null,
  content varchar(2048),
  created_at timestamp not null default current_timestamp,
  updated_at timestamp not null default current_timestamp,
  deleted_at timestamp default null,
  constraint pk_messages primary key (id),
  constraint fk_messages_sender foreign key (sender_id)
  references users(id),
  constraint fk_messages_receiver foreign key (receiver_id)
  references users(id)
);
