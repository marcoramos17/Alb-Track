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
    const enriched = await res.json();

    renderUsers(enriched);
}

function renderUsers(enriched) {
    usersDiv.innerHTML = "";

    const users = enriched.users;
    const churches = enriched.churches;
    const albs = enriched.albs;

    users.forEach(user => {
        const div = document.createElement("div");

        // Backend already filtered these
        const userChurches = churches;
        const userAlbs = albs;

        let relationHTML = "";

        userChurches.forEach(church => {
            relationHTML += `<strong>Church:</strong> ${church.church_name}<br>`;

            // Match albs to this church
            const albsForChurch = userAlbs.filter(a => a.church_id === church.church_id);

            if (albsForChurch.length === 0) {
                relationHTML += `&nbsp;&nbsp;Alb: <em>None</em><br>`;
            } else {
                albsForChurch.forEach(alb => {
                    relationHTML += `&nbsp;&nbsp;Alb: ${alb.alb_code} (${alb.alb_size}cm)<br>`;
                });
            }

            relationHTML += "<br>";
        });

        // Personal albs (no church)
        const personalAlbs = userAlbs.filter(a => a.church_id === null);
        if (personalAlbs.length > 0) {
            relationHTML += `<strong>Personal Albs:</strong><br>`;
            personalAlbs.forEach(alb => {
                relationHTML += `&nbsp;&nbsp;Alb: ${alb.alb_code} (${alb.alb_size}cm)<br>`;
            });
        }

        div.innerHTML = `
            <strong>${user.first_name} ${user.last_name}</strong><br>
            Phone: ${user.phone ?? ""}<br>
            Birth Date: ${user.birth_date}<br>
            Active: ${user.active ? "Yes" : "No"}<br>
            <img src="http://localhost:3000/photos/${user.user_id}.jpg" width="120"><br><br>

            ${relationHTML}
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
