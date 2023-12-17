select * 
from integrantes i, oficinas o, presenca pre
where pre.id_oficina = o.id_oficina
and i.id_integrante = pre.id_integrante;