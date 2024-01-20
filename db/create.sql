
create table integrantes (
	id_integrante serial primary key,
	email varchar(50),
	nome varchar(50),
	sobrenome varchar(50),
	senha varchar(50)
);

create table oficinas (
	titulo varchar(50),
	id_oficina serial primary key,
	link_gravacao varchar(100),
	markdown varchar(10000),
	id_autor int,
	data_oficina date,
	foreign key (id_autor) references integrantes(id_integrante)
);

create table problemas (
	id_oficina int,
	alias varchar(50),
	link_problema varchar(100),
	foreign key (id_oficina) references oficinas(id_oficina),
	primary key (id_oficina, alias)
);

create table presenca (
	id_integrante int,
	id_oficina int,
	foreign key (id_integrante) references integrantes(id_integrante),
	foreign key (id_oficina) references oficinas(id_oficina),
	primary key (id_integrante, id_oficina)
);