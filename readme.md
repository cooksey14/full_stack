# Colin's Full stack rust application

To run this application you need to start the front and backend seperately. To do this you can run the following commands:

-Backend: `<cargo +nightly run -p backend --bin backend>`\
    -This needs to be executed from the root directory\
-Frontend: In terminal run `<microserver>`\
    -This needs to be execute from the `</fullstack/frontend>` directory


To create a new task:
`<cargo +nightly run -p backend --bin todo new '<task_here>'`


# Think About Structure

Who depends on who?

FrontEnd -> Backend

Backend has Task and JSONApiTaskResponse structure.

Backend Defines your Data and Operations
Frontend Consumes the Backend Data and Opeations

## Backend [bin]
Defines
    Task
    JSONApiTaskResponse
    
Has a main.rs

# Frontend [bin]
Frontend Uses:
    backend::Task
    backend::JSONApiTaskResponse
    
Has a main.rs

Move sqlite3 files? out of the source code.

