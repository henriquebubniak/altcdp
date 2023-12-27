insert into integrantes (email, nome, sobrenome, senha)
values ('hg.bubniak@gmail.com', 'henrique', 'bubniak', 'abacate123');

insert into integrantes (email, nome, sobrenome, senha)
values ('zambeta@gmail.com', 'henrique', 'zambenedetti', 'abacaxi123');

insert into integrantes (email, nome, sobrenome, senha)
values ('spadas@gmail.com', 'gabriel', 'spadafora', 'manga123');

insert into oficinas (titulo, link_gravacao, id_autor, data_oficina)
values ('Dijkstra','googlemeet.com', 1, '2023-12-12');

insert into oficinas (titulo, link_gravacao, id_autor, data_oficina)
values ('Programação Dinamica','googlemeet.com', 2, '2023-12-13');

insert into oficinas (titulo, link_gravacao, id_autor, data_oficina)
values ('Algoritmos gulosos','googlemeet.com', 2, '2023-12-13');

insert into problemas (id_oficina, link_problema, alias)
values (1, 'atcoder.com', 'ATC123A');

insert into problemas (id_oficina, link_problema, alias)
values (1, 'codeforces.com', 'CF123B');

insert into problemas (id_oficina, link_problema, alias)
values (2, 'timus.com', 'TI123C');

insert into problemas (id_oficina, link_problema, alias)
values (2, 'beecrowd.com', 'BC123D');

insert into presenca (id_integrante, id_oficina)
values (1, 1);

insert into presenca (id_integrante, id_oficina)
values (2, 1);

insert into presenca (id_integrante, id_oficina)
values (2, 2);

insert into presenca (id_integrante, id_oficina)
values (3, 2);