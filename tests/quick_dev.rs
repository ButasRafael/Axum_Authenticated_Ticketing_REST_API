use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc=httpc_test::new_client("http://localhost:8081")?;
    hc.do_get("/hello?name=Diana").await?.print().await?;
    hc.do_get("/hello2/Rafa").await?.print().await?;
    hc.do_get("/src/main.rs").await?.print().await?;
    let req_login=hc.do_post("/api/login",json!({
        "username":"admin",
        "password":"welcome"
    })
    );
    req_login.await?.print().await?;
    let req_create_ticket=hc.do_post("/api/ticket",json!({
        "title":"Ticket 1"
    })
    );
    req_create_ticket.await?.print().await?;
    let req_create_ticket=hc.do_post("/api/ticket",json!({
        "title":"Ticket 2"
    })
    );
    req_create_ticket.await?.print().await?;
    let req_list_tickets=hc.do_get("/api/ticket");
    req_list_tickets.await?.print().await?;
    let req_get_ticket=hc.do_get("/api/ticket/0");
    req_get_ticket.await?.print().await?;
    let req_update_ticket=hc.do_put("/api/ticket/0",json!({
        "title":"Ticket 1 - Updated"
    })
    );
    req_update_ticket.await?.print().await?;
    let req_list_tickets=hc.do_get("/api/ticket");
    req_list_tickets.await?.print().await?;
    let req_delete_ticket=hc.do_delete("/api/ticket/0");
    req_delete_ticket.await?.print().await?;
    let req_list_tickets=hc.do_get("/api/ticket");
    req_list_tickets.await?.print().await?;
    let req_get_ticket=hc.do_get("/api/ticket/0");
    req_get_ticket.await?.print().await?;
    Ok(())

}
