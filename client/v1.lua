function main()
    turtle = {}
    turtle.x = 0
    turtle.y = 0
    turtle.z = 0
    turtle.fuel = 0
    turtle.items = {}
    id = 0
    reg = "{\"WebReg\":["..id..",{\"pos\":["..turtle.x..","..turtle.y..","..turtle.z.."],\"items\":"..turtle.items..",\"fuel\":"..turtle.fuel..",\"recv\":[]}]}"
    print(http.post("http://127.0.0.1:8000/register",reg))
end

main()