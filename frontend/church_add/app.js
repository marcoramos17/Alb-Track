document.getElementById("add-church-form").addEventListener("submit", async (e) => {
    e.preventDefault();

    const form = document.getElementById("add-church-form");
    const formData = new FormData(form);

    const res = await fetch("http://localhost:3000/api/church", {
        method: "POST",
        body: formData
    });

    console.log(await res.text());
});
