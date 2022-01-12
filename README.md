# Shopify Developer Intern Challenge

An inventory tracking web app for a hypothetical logistics company. It has a CRUD interface for managing the inventory
items. Inventory items can also be added to shipments which can be "sent out". Once a shipment is sent out, the sent
inventory is automatically removed from the respective items.

### Running

To run this application, you will need to install Docker and Docker Compose. Instructions for each can be found
[here](https://docs.docker.com/get-docker/) for Docker, and [here](https://docs.docker.com/compose/install/) for Docker
Compose.

Once installed, simply open a terminal in the project directory and run:
```shell
docker-compose up
```

This will set up the application itself, along with a PostgreSQL database. The application will be accessible at
`http://127.0.0.1:3000`.

### Endpoints

The following endpoints exist in this API:
- Inventory
  - `GET` `/inventory` - get a list of all the items in the inventory
  - `POST` `/inventory` - create a new item from a JSON body with the following parameters
    - `name`: the name of the item
    - `description`: an optional description for the item
    - `stock`: how much is present in the inventory
  - `PUT` `/inventory/:id` - edit an item by its ID with a JSON body accepting the same parameters as creating an item
  - `DELETE` `/inventory/:id` - delete an item by its ID
- Shipments
  - `GET` `/shipments` - get a list of all shipments
  - `POST` `/shipments` - create a new shipment from a JSON body with the following parameters
    - `name`: a human-readable name for the shipment
  - `GET` `/shipments/:id` - get all the inventory that will be "sent" in a shipment
  - `PUT` `/shipments/:id/items` - Add or modify the amount of an item by its ID in the shipment with a JSON body
    - `item_id`: the ID of an item in the inventory
    - `count`: the amount of the item to "ship"
  - `DELETE` `/shipments/:id/items/:item_id` - Remove an item from the shipment
  - `DELETE` `/shipment/:id` - delete a shipment by its ID
  - `POST` `/shipments/:id/ship` - "send" out a shipment by its ID

There are also collections for [Insomnia](https://insomnia.rest/products/insomnia) and [Postman](https://www.postman.com/)
that can be imported for ease of use. Instructions for importing can be found here for 
[Insomnia](https://docs.insomnia.rest/insomnia/import-export-data#import-data), and here for 
[Postman](https://learning.postman.com/docs/getting-started/importing-and-exporting-data/#importing-postman-data).

The Insomnia collection is called [`insomnia.json`](./insomnia.json) and the Postman collection is called 
[`postman.json`](./postman.json).
