# Shops

Purchase and sell interface for trading currency for items and services.

## Core Logic

**Concept**

- Entities can offer goods for sale
- Players spend currency to purchase
- Optional: players sell items back
- Stock, pricing, availability configurable

**Shop Properties**

| Property | Type | Description |
|----------|------|-------------|
| id | string | Shop identifier |
| stock | list | Available items |
| currency | string | Accepted currency type |
| buy_multiplier | float | Sell price modifier |
| restock | enum | Restock behavior |

**Stock Entry**

| Property | Type | Description |
|----------|------|-------------|
| item_id | string | What's for sale |
| price | int | Cost in currency |
| quantity | int | Available stock (-1 = infinite) |
| requirements | list | Unlock conditions |

**Operations**

`get_stock(shop_id)` - List available items

- Return filtered by requirements
- Include prices and quantities

`purchase(shop_id, item_id, quantity)` - Buy item

- Check currency sufficient
- Check stock available
- Deduct currency, grant item
- Reduce stock

`sell(shop_id, item_id, quantity)` - Sell item

- Check shop accepts sells
- Calculate buy price
- Remove item, grant currency

`restock(shop_id)` - Refresh stock

- Reset quantities
- May randomize stock

**Invariants**

- Can't purchase without sufficient currency
- Can't purchase out-of-stock items
- Prices consistent within transaction
- Stock updates atomically

**Design Notes**

- Specific shops left to design
- Pricing left to design
- Restock timing left to design

---

## Bevy Integration

**Data**

- ShopDefinition { id, stock, currency, buy_multiplier, restock }
- ShopState { current_stock: HashMap<ItemId, u32> }

**Messages/Commands**

- OpenShop { shop_id }
- Purchase { shop_id, item_id, quantity }
- Sell { shop_id, item_id, quantity }
- RestockShop { shop_id }

**Events**

- ShopOpened { shop_id }
- ItemPurchased { shop_id, item_id, quantity, price }
- ItemSold { shop_id, item_id, quantity, price }

**UI**

- Shop interface displays stock
- Player inventory for selling
- Currency balance visible

**Scripting Compatibility**

- Purchase/sell as commands
- Stock readable
- Events hookable

*See: architecture/scripting.md, architecture/data.md*
