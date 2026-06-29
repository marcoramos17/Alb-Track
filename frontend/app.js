document.getElementById("add-server-form").addEventListener("submit", async (e) => {
    e.preventDefault();

    const payload = {
        first_name: document.getElementById("first_name").value,
        last_name: document.getElementById("last_name").value,
        phone: document.getElementById("phone").value,
        birth_date: document.getElementById("birth_date").value,
        password: document.getElementById("password").value
    };

    await fetch("http://localhost:3000/api/users", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(payload)
    })
});
