document.getElementById("add-alb-form").addEventListener("submit", async (e) => {
    e.preventDefault();

    const form = document.getElementById("add-alb-form");
    const formData = new FormData(form);

    const res = await fetch("http://localhost:3000/api/albs", {
        method: "POST",
        body: formData
    });

    console.log(await res.text());
});


// Dynamic dropdown logic
const adultSelect = document.getElementById("adult_alb");
const accessorySelect = document.getElementById("has_accessory");
const accessoryLabel = document.querySelector("label[for='has_accessory']");

// Hide accessory field initially
accessorySelect.style.display = "none";
accessoryLabel.style.display = "none";

adultSelect.addEventListener("change", () => {
    // Show accessory field once adult/child is chosen
    accessorySelect.style.display = "block";
    accessoryLabel.style.display = "block";

    // Change label dynamically
    if (adultSelect.value === "0") {
        accessoryLabel.textContent = "Has Surplice:";
    } else {
        accessoryLabel.textContent = "Has Cingulum:";
    }
});
