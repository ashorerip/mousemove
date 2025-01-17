local mousemove = function(type, x, y)
    assert(typeof(x) == "number", "x not a number");
    assert(typeof(y) == "number", "y not a number");

    http_request({
        Method = "POST",
        Body = game:GetService("HttpService"):JSONEncode({
            x, y
        }),
        Url = `http://localhost:8080/mousemove{type}`
    });
end

getgenv().mousemoverel = function(x, y)
    mousemove("rel", x, y);
end

getgenv().mousemoveabs = function(x, y)
    mousemove("abs", x, y);
end