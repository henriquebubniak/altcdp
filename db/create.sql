

create table participants (
	participant_id serial primary key,
	email varchar(50),
	first_name varchar(50),
	last_name varchar(50)
	
);

create table workshops (
	title varchar(50),
	workshop_id serial primary key,
	recording_link varchar(50),
	author_id int,
	workshop_date varchar(50),
	foreign key (author_id) references participants(participant_id)
);

create table problems (
	workshop_id int,
	problem_link varchar(50),
	foreign key (workshop_id) references workshops(workshop_id)	
);

create table presence (
	participant_id int,
	workshop_id int,
	foreign key (participant_id) references participants(participant_id),
	foreign key (workshop_id) references workshops(workshop_id)	
);