### Architecture Overview of RAT

**Presentation Layer:**

- Routing is done here to match an HTTP request to the correct function
- Responsible for deserializing requests and serializing responses.
- Owns models like HTML templates or structures for encoding in JSON/XML.
- Encapsulates encoding details for server responses.
- Calls the services layer.

**Services Layer:**

- Houses the application's business logic, rules, and invariants.
- Manages tasks such as defining validation rules for creating jobs for agents.
- Contains the answers to questions regarding application behavior and rules.

**Entities Layer:**

- Contains structures used by the services layer.
- Each service has its own set of entities.
- Differs from a traditional model as it encompasses all structures, not just those persisted in a database or transmitted by the presentation layer.
- Examples include Agent and Job (representing a command created, stored, dispatched, and executed within the application).

**Repository Layer:**

- Acts as a thin abstraction over the database, handling all database calls.
- Called by the services layer to interact with the database.
- Encapsulates database access to ensure separation of concerns and maintainability.

**Drivers:**

- Handles calls to third-party APIs and communication with external services like email servers or block storage.
- Can only be invoked by the services layer, as it directly interacts with external systems.
- Responsible for managing interactions with external dependencies while keeping the business logic isolated within the services layer.
