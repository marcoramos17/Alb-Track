# Computer_Science_project-template
<div align="center">
  <!-- GitHub Profile Shield with Logo -->
  <a href="https://github.com/marcoramos17">
    <img src="https://img.shields.io/badge/GitHub-Profile-181717?logo=github" alt="{GitHub} Profile"></a>
</div>



# Affiliation - Project (Template)

This repository serves as a centralized template for creating and managing project repositories efficiently, with support for kanban boards and other basic planning features.

---

## Document Information

### Affiliation:
*Example University*

### Project:
*Module/Project Name*

### Title:
*Document Title Here*

### Author(s):
*Marco Ramos - 10415201*

### Description:
This repository provides a well-structured template for computer science projects.

---

## Workflow Guide

### **Working with the Main Branch**
The `main` branch:
- Repository main branch

#### Key Actions:
- **Update the repository (`main` branch):**
  ```bash
  git add .
  git commit -m "message"
  git push
  ```

---

### **Creating a New Branch**
To start working on a new branch:

1. **Create a new branch OR switch to an existing branch:**
   ```bash
   git checkout -b branch-name
   #OR
   git checkout branch-name
   ```
   - Branch naming format: `-- no format specified --`. Ex: `-- branch_name --`

2. **Push the branch to the remote repository (optional):**
   ```bash
   git push
   ```

---

### **Keeping The Other Branches Updated**
If updates are made to the `main` branch, pull those changes into other branches:

1. **Switch to another document branch:**
   ```bash
   git checkout branch-name
   ```

2. **Pull changes from `main`:**
   ```bash
   git pull origin main
   ```

3. **Resolve any merge conflicts (if applicable):**
   Open conflicting files, resolve issues manually, and mark them resolved:
   ```bash
   git add resolved-file
   git commit
   ```

---

### **General Tips**
- Regularly pull updates from `main` to keep branches up-to-date with the latest template changes.
- Use descriptive branch names (e.g., `COV_6006CEM_Train-Accidents`).
- Write clear commit messages for better collaboration and tracking.

---

This README serves as an introduction to maintain an organized and efficient workflow for managing Computer Science Projects.
