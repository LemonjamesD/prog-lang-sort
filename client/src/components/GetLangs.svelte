<script lang="ts">

function send_get() {
    const url: String = "http://localhost:8080/echo";
    const data: JSON = {
      "data": "Hello, World!"
    };

    var http_req: XMLHttpRequest = new XMLHttpRequest();
    http_req.onreadystatechange = () => { 
        if (http_req.readyState == 4 && http_req.status == 200)
            console.log(http_req.response);
        else if (http_req.readState == 4)
            console.log("Could not fetch the data");
    }
    http_req.open("POST", url);
    http_req.send(JSON.stringify(data));
    console.log(http_req);
}

</script>

<input on:click={send_get} type="button" value="Button"/>
