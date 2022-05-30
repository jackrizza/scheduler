## Scheduler

### Abstract
The general idea of Scheduler is to have a application spawn three threads that have accsess to the op log. 

The first thread `executer` will loop over the first oplog and check for event execution. If the event has not been executed it will execute the event otherwise, it will remove the event from the oplog.

The second thread `lisenter` will wait for user input and insert new event into oplog

the third thread `cleaner` will run over oplog and reprioritize events in oplog based on how far away they are from `epoch`
