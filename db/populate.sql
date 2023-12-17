insert into integrantes (email, nome)
values ('hg.bubniak@gmail.com', 'henrique bubniak');

insert into integrantes (email, nome)
values ('zambeta@gmail.com', 'henrique zambenedetti');

insert into integrantes (email, nome)
values ('spadas@gmail.com', 'gabriel spadafora');

insert into oficinas (titulo, link_gravacao, id_autor, data_oficina)
values ('Dijkstra','googlemeet.com', 1, '2023-12-12');

insert into oficinas (titulo, link_gravacao, id_autor, data_oficina)
values ('Programação Dinamica','googlemeet.com', 2, '2023-12-13');

insert into oficinas (titulo, link_gravacao, id_autor, data_oficina)
values ('Algoritmos gulosos','googlemeet.com', 2, '2023-12-13');

insert into problemas (id_oficina, link_problema)
values (1, 'atcoder.com');

insert into problemas (id_oficina, link_problema)
values (1, 'codeforces.com');

insert into problemas (id_oficina, link_problema)
values (2, 'timus.com');

insert into problemas (id_oficina, link_problema)
values (2, 'beecrowd.com');

insert into presenca (id_integrante, id_oficina)
values (1, 1);

insert into presenca (id_integrante, id_oficina)
values (2, 1);

insert into presenca (id_integrante, id_oficina)
values (2, 2);

insert into presenca (id_integrante, id_oficina)
values (3, 2);