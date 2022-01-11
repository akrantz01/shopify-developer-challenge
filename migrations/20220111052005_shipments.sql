CREATE TABLE IF NOT EXISTS shipments (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS shipment_inventory (
    shipment_id INTEGER NOT NULL,
    item_id INTEGER NOT NULL,
    count INTEGER NOT NULL,
    PRIMARY KEY (shipment_id, item_id),
    FOREIGN KEY(shipment_id) REFERENCES shipments(id) ON DELETE CASCADE,
    FOREIGN KEY(item_id) REFERENCES inventory(id) ON DELETE CASCADE
)
