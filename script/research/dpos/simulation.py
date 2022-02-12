import time
from ouroboros import Stakeholder
from ouroboros import Z

EPOCH_LENGTH = 2
stakeholders = []

for i in range(2):
    stakeholders.append(Stakeholder(EPOCH_LENGTH))

stakeholders[0].set_leader()
environment = Z(stakeholders, EPOCH_LENGTH, genesis_time=time.time())
environment.start()

for sh in environment.stakeholders:
    sh.beacon.join()
