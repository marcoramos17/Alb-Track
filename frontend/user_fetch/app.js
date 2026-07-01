const API_URL = "http://localhost:3000/api/users";

const searchInput = document.getElementById("search");
const activeFilter = document.getElementById("activeFilter");
const usersDiv = document.getElementById("users");

async function loadUsers() {
    const params = new URLSearchParams();

    // Search
    const search = searchInput.value.trim();

    if (search !== "") {
        if (search.includes(" ")) {
            // Full name search
            const [first, last] = search.split(" ", 2);

            params.append("first_name", first);
            params.append("last_name", last);
        } else {
            // Single term search
            params.append("first_name", search);
            params.append("last_name", search);
        }
    }


    // Active filter
    if (activeFilter.value === "active") {
        params.append("active", "true");
    } else if (activeFilter.value === "inactive") {
        params.append("active", "false");
    }

    const res = await fetch(`${API_URL}?${params.toString()}`);
    const users = await res.json();

    renderUsers(users);
}

function renderUsers(users) {
    usersDiv.innerHTML = "";

    users.forEach(u => {
        const div = document.createElement("div");

        div.innerHTML = `
            <strong>${u.first_name} ${u.last_name}</strong><br>
            Phone: ${u.phone ?? ""}<br>
            Birth Date: ${u.birth_date}<br>
            Active: ${u.active ? "Yes" : "No"}<br>
            <img src="http://localhost:3000/photos/${u.user_id}.jpg" width="120">
            <hr>
        `;

        usersDiv.appendChild(div);
    });
}

// Auto-refresh on input changes
searchInput.addEventListener("input", loadUsers);
activeFilter.addEventListener("change", loadUsers);

// Initial load
loadUsers();
