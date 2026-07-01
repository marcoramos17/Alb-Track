document.getElementById("add-server-form").addEventListener("submit", async (e) => {
    e.preventDefault();

    const form = document.getElementById("add-server-form");
    const formData = new FormData(form);

    const res = await fetch("http://localhost:3000/api/users", {
        method: "POST",
        body: formData
    });

    console.log(await res.text());
});
