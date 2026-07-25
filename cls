[33mcommit 1a464ac4b99afef49e343ed97218784183545fe0[m[33m ([m[1;36mHEAD[m[33m -> [m[1;32mmaster[m[33m)[m
Author: Habib Azargoshasb <h.azargoshasb85@gmail.com>
Date:   Sat Jul 25 10:28:15 2026 +0330

    Complete book CRUD: implement RemoveBook, EditBook, SearchBook, and fix critical bug with duplicate book IDs via next_book_id counter

[33mcommit 256d69eee00746fcb4e4bb713ddafde93f81b9ab[m
Author: Habib Azargoshasb <h.azargoshasb85@gmail.com>
Date:   Thu Jul 23 10:13:00 2026 +0330

    Implement LibraryState with Arc<Mutex>, process AddBook and ListBooks requests

[33mcommit 31578e2f05dacb7991ece355ceeda47b26c6220a[m
Author: Habib Azargoshasb <h.azargoshasb85@gmail.com>
Date:   Tue Jul 21 10:00:05 2026 +0330

    Add basic client, implement Request serialization and server-side deserialization over TCP

[33mcommit b28b7c830cbfeea45c4672895003f260532cf398[m
Author: Habib Azargoshasb <h.azargoshasb85@gmail.com>
Date:   Mon Jul 20 12:45:54 2026 +0330

    Implement basic TCP server skeleton with connection acceptance and per-client task spawning

[33mcommit 356814a0ffc45e8bf2c430f6e5b4c1ddeaa5f97a[m
Author: Habib Azargoshasb <h.azargoshasb85@gmail.com>
Date:   Mon Jul 20 12:24:56 2026 +0330

    Add tokio dependencyto server crate

[33mcommit b02721ff47f0953229b3c8862ca2019809135453[m
Author: Habib Azargoshasb <h.azargoshasb85@gmail.com>
Date:   Mon Jul 20 10:07:14 2026 +0330

    Complete shared crate: Book, Member, Loan, Reservation, Fine, Rating, Request/Response messages

[33mcommit 6e7c92e748397129e83ab82ce4d5f34da9d054cc[m
Author: Habib Azargoshasb <h.azargoshasb85@gmail.com>
Date:   Mon Jul 20 09:05:26 2026 +0330

    Initial commit: workspace structure with server, client, and shared crates
