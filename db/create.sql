

create table integrantes (
	id_integrante serial primary key,
	email varchar(50),
	nome varchar(50),
	senha varchar(50),
);

create table oficinas (
	titulo varchar(50),
	id_oficina serial primary key,
	link_gravacao varchar(50),
	id_autor int,
	data_oficina varchar(50),
	foreign key (id_autor) references integrantes(id_integrante)
);

create table problemas (
	id_oficina int,
	link_problema varchar(50),
	foreign key (id_oficina) references oficinas(id_oficina)	
);

create table presenca (
	id_integrante int,
	id_oficina int,
	foreign key (id_integrante) references integrantes(id_integrante),
	foreign key (id_oficina) references oficinas(id_oficina)	
);