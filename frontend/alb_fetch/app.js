const API_URL = "http://localhost:3000/api/albs";

const searchInput = document.getElementById("search");
const adultFilter = document.getElementById("adultFilter");
const accessoryFilter = document.getElementById("accessoryFilter");
const accessoryLabel = document.getElementById("accessoryLabel");
const albsDiv = document.getElementById("albs");

async function loadAlbs() {
    const params = new URLSearchParams();

    // Search by code or size
    const search = searchInput.value.trim();
    if (search !== "") {
        params.append("alb_code", search);
    }

    // Adult/child filter
    if (adultFilter.value !== "all") {
        params.append("adult_alb", adultFilter.value);
    }

    // Accessory filter
    if (accessoryFilter.value !== "all") {
        params.append("has_accessory", accessoryFilter.value);
    }

    const res = await fetch(`${API_URL}?${params.toString()}`);
    const albs = await res.json();

    renderAlbs(albs);
}

function renderAlbs(albs) {
    albsDiv.innerHTML = "";

    albs.forEach(a => {
        const div = document.createElement("div");

        div.innerHTML = `
            <strong>${a.alb_code}</strong><br>
            Size: ${a.alb_size}<br>
            Type: ${a.adult_alb ? "Adult" : "Child"}<br>
            Accessory: ${a.has_accessory ? "Yes" : "No"}<br>
            Notes: ${a.notes ?? ""}<br>
            <hr>
        `;

        albsDiv.appendChild(div);
    });
}

// Dynamic accessory label
adultFilter.addEventListener("change", () => {
    if (adultFilter.value === "0") {
        accessoryLabel.textContent = "Has Surplice:";
    } else if (adultFilter.value === "1") {
        accessoryLabel.textContent = "Has Cingulum:";
    } else {
        accessoryLabel.textContent = "Has Accessory:";
    }
});

// Auto-refresh
searchInput.addEventListener("input", loadAlbs);
adultFilter.addEventListener("change", loadAlbs);
accessoryFilter.addEventListener("change", loadAlbs);

// Initial load
loadAlbs();
