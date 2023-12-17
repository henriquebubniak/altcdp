insert into participants (email, first_name, last_name)
values ('hg.bubniak@gmail.com', 'henrique', 'bubniak');

insert into participants (email, first_name, last_name)
values ('zambeta@gmail.com', 'henrique', 'zambenedetti');

insert into participants (email, first_name, last_name)
values ('spadas@gmail.com', 'gabriel', 'spadafora');

insert into workshops (title, recording_link, author_id, workshop_date)
values ('Dijkstra','googlemeet.com', 1, '2023-12-12');

insert into workshops (title, recording_link, author_id, workshop_date)
values ('Programação Dinamica','googlemeet.com', 2, '2023-12-13');

insert into workshops (title, recording_link, author_id, workshop_date)
values ('Algoritmos gulosos','googlemeet.com', 2, '2023-12-13');

insert into problems (workshop_id, problem_link)
values (1, 'atcoder.com');

insert into problems (workshop_id, problem_link)
values (1, 'codeforces.com');

insert into problems (workshop_id, problem_link)
values (2, 'timus.com');

insert into problems (workshop_id, problem_link)
values (2, 'beecrowd.com');

insert into presence (participant_id, workshop_id)
values (1, 1);

insert into presence (participant_id, workshop_id)
values (2, 1);

insert into presence (participant_id, workshop_id)
values (2, 2);

insert into presence (participant_id, workshop_id)
values (3, 2);