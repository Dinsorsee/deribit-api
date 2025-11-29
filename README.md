# Requirement & Scope

> This project targets Dollar Cost Averaging at the Minimum Viable Product (MVP) stage. To keep the project
> scalable and simple, we will design it up to the point where a buy transaction is executed and recorded in the database.

- We can connect with Deribit Testnet to obtain ‘access_token’ and ‘refresh_token’ through the deribit API.
- We can fetch market data, including asset names and prices.
- We can fetch the account summary.
- The system can calculate the transaction amount based on market data and account summary.
- The system can automatically authenticate and simulate a one-time DCA order using the /POST logic.
- A transaction will be inserted into the database after the order has been successfully executed.

## Out of scope

- Future enhancements for full DCA automation will be developed later.
- Using WebSocket or FIX instead of REST for latency best practices: For frequent orders or real-time execution speed, prefer WebSocket or FIX. REST is suitable only for infrequent requests or when simplicity is more important than speed.

### References

- [This is an external link to rust-book.cs.brown.edu](https://rust-book.cs.brown.edu/ch05-03-method-syntax.html)
- [This is an external link to doc.deribit.com](https://docs.deribit.com/#deribit-api-v2-1-1)
- [This is an external link to dhghomon.github.io](https://dhghomon.github.io/easy_rust/Chapter_25.html)
- [This is an external link to github.com/dovahcrow](https://github.com/dovahcrow/deribit-rs/tree/master)
