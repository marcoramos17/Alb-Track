const API_URL = "http://localhost:3000/api/church";

const searchInput = document.getElementById("search");
const churchesDiv = document.getElementById("churches");

async function loadChurches() {
    const params = new URLSearchParams();

    const search = searchInput.value.trim();
    if (search !== "") {
        params.append("church_name", search);
    }

    const res = await fetch(`${API_URL}?${params.toString()}`);
    const churches = await res.json();

    renderChurches(churches);
}

function renderChurches(churches) {
    churchesDiv.innerHTML = "";

    churches.forEach(c => {
        const div = document.createElement("div");

        div.innerHTML = `
            <strong>${c.church_name}</strong><br>
            ID: ${c.church_id}
            <hr>
        `;

        churchesDiv.appendChild(div);
    });
}

searchInput.addEventListener("input", loadChurches);
loadChurches();
