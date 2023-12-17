select * 
from participants p2, workshops w, presence pre
where pre.workshop_id = w.workshop_id
and p2.participant_id = pre.participant_id;