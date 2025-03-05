# 📋 Clipboard Manager

A simple clipboard manager that **runs in the background** and **saves copied text to a file** instead of losing it when you copy something new.

## ✨ Features
- 🖥️ **Runs in the background**  
- 📋 **Automatically saves copied text**  
- 💾 **Stores clipboard history in a file (`clipboard_history.txt`)**  
- 🚀 **Lightweight and fast**  

---

## 🚀 How It Works
1. The application **monitors your clipboard** every second.
2. When **new text is copied**, it gets **saved in `clipboard_history.txt`**.
3. The app **ignores duplicate copies** to prevent unnecessary writes.

---

## 🔧 Installation & Usage

### **🔹 Running the Program**
#### **1️⃣ Download & Install Rust** (if not already installed)  
[Get Rust here](https://www.rust-lang.org/) and install it.  

#### **2️⃣ Clone This Repository**
```sh
git clone https://github.com/YOUR_USERNAME/ClipboardManager.git
cd ClipboardManager
