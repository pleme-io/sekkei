//! Shared test fixtures for the sekkei crate.
//!
//! Centralises the canonical spec YAML used across multiple test modules,
//! eliminating duplication between `types::tests` and `visitor::tests`.

/// Minimal OpenAPI 3.0.3 spec in YAML for testing.
pub const MINIMAL_SPEC_YAML: &str = r#"
info:
  title: Test API
  version: "1.0.0"
paths: {}
"#;

/// A richer spec with components, paths, servers, and security schemes.
pub const FULL_SPEC_YAML: &str = r##"
info:
  title: Pet Store
  description: A sample pet store API
  version: "2.0.0"
servers:
  - url: https://api.petstore.example.com/v2
    description: Production server
security:
  - bearerAuth: []
paths:
  /pets:
    get:
      operationId: listPets
      summary: List all pets
      parameters:
        - name: limit
          in: query
          required: false
          schema:
            type: integer
            format: int64
      responses:
        "200":
          description: A list of pets
          content:
            application/json:
              schema:
                type: array
                items:
                  $ref: "#/components/schemas/Pet"
    post:
      operationId: createPet
      summary: Create a pet
      requestBody:
        required: true
        content:
          application/json:
            schema:
              $ref: "#/components/schemas/CreatePetRequest"
      responses:
        "201":
          description: Pet created
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/Pet"
  /pets/{petId}:
    parameters:
      - name: petId
        in: path
        required: true
        schema:
          type: string
    get:
      operationId: getPet
      summary: Get a pet by ID
      responses:
        "200":
          description: A pet
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/Pet"
        "404":
          description: Pet not found
    delete:
      operationId: deletePet
      summary: Delete a pet
      responses:
        "204":
          description: Pet deleted
components:
  schemas:
    Pet:
      type: object
      required:
        - id
        - name
      properties:
        id:
          type: integer
          format: int64
        name:
          type: string
        tag:
          type: string
        status:
          $ref: "#/components/schemas/PetStatus"
    PetStatus:
      type: string
      enum:
        - available
        - pending
        - sold
    CreatePetRequest:
      type: object
      required:
        - name
      properties:
        name:
          type: string
          description: The pet's name
        tag:
          type: string
          description: Optional tag
  securitySchemes:
    bearerAuth:
      type: http
      scheme: bearer
  parameters:
    LimitParam:
      name: limit
      in: query
      required: false
      schema:
        type: integer
  requestBodies:
    PetBody:
      required: true
      content:
        application/json:
          schema:
            $ref: "#/components/schemas/CreatePetRequest"
  responses:
    NotFound:
      description: The requested resource was not found
"##;
