# Benchmarking

Performance was benchmarked using criterion by testing input sizes of 1e2, 1e3, 1e4, 1e5, 1e6, 1e7, and 1e8 against a threadpool with 1,2, 4, 8, 16, 32, 64, 128, and 256 threads. 

![Screenshot 2024-11-03 at 1 07 20 AM](https://github.com/user-attachments/assets/900b9106-32b4-42f9-b5c6-89908cdb6b54)
For small arrays of less than 1e2 we observe a performance decrease for all threads greater than 1. This is because the overhead in scheduling is more drastic than the few hundred cycles needed to actually sort the array. 

![Screenshot 2024-11-03 at 1 08 36 AM](https://github.com/user-attachments/assets/94ca5056-fc16-482b-883a-8921c9609a8b)
For arrays at size around 1e5 we see some very nice improvements from adding more threads. The performance increase is almost 2x going from 1 threads to 2 threads and continues dropping almost linearly up to 8 threads. Going from 8-256 threads the performance reverses and quickly begings to deteriorate. My macbook has only 8 logical cores which are fully CPU bound at 8 threads so any higher threadcount just add additional overhead with no gain. 

![Screenshot 2024-11-03 at 1 11 56 AM](https://github.com/user-attachments/assets/d972f44c-02d5-43e8-930c-b4d7e54df229)
For really massive arrays >=1e8 the performance again tops out at 8 threads since we are fully CPU bound but it mostly stays leveled off without significant degredation from adding more threads. We fully max out the cpu and keep large chunks of works regardless of threadcount so it becomes completely irrelevent. 
