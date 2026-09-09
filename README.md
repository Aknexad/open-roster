


# Standard Production-Ready

openmember/
├── Cargo.toml
├── build.rs                 # کامپایل کدهای .slint به Rust (slint-build)
├── ui/                      # تمام فایل‌های واسط کاربری Slint
│   ├── appwindow.slint      # پنجره اصلی برنامه (Main Window)
│   ├── styles/              # رنگ‌ها، تایپوگرافی، تم دارک/لایت، آیکون‌ها
│   │   ├── colors.slint
│   │   ├── typography.slint
│   │   └── theme.slint
│   ├── components/          # کامپوننت‌های عمومی و قابل استفاده مجدد
│   │   ├── button.slint
│   │   ├── input.slint
│   │   ├── badge.slint      # بج وضعیت (Active, Expiring Soon, Expired)
│   │   ├── modal.slint
│   │   ├── toast.slint
│   │   └── data_table.slint
│   └── views/               # صفحات و ویوهای مختلف
│       ├── dashboard.slint   # داشبورد آماری و هشدارهای انقضا
│       ├── members.slint     # لیست، جستجو، فیلتر و فرم ثبت/ویرایش عضو
│       ├── plans.slint       # مدیریت پلن‌های عضویت
│       └── settings.slint    # تنظیمات، بکاپ و اتصال Google Drive
│
├── assets/                  # فونت‌ها، آیکون‌ها، تصاویر و لوگوی نرم‌افزار
│   ├── fonts/               # برای پشتیبانی فارسی و اعداد
│   └── icons/
│
├── src/                     # کد سمت Rust (منطق، دیتابیس، بایندر Slint)
│   ├── main.rs              # نقطه ورود، راه‌اندازی UI و اتصال Event Loop
│   ├── app.rs               # کنترلر اصلی و سیم‌کشی پل ارتباطی بین Slint و Rust
│   │
│   ├── models/              # ساختار داده‌ها (Data Structs & Enums)
│   │   ├── mod.rs
│   │   ├── member.rs        # فیلدهای Member، وضعیت MemberStatus
│   │   ├── plan.rs          # ساختار پلن‌های اشتراک
│   │   └── backup.rs        # متادیتای بکاپ و وضعیت سینک
│   │
│   ├── db/                  # لایه دیتابیس و انبارش (SQLite)
│   │   ├── mod.rs
│   │   ├── connection.rs    # راه‌اندازی SQLite در حالت WAL
│   │   ├── migrations/      # اسکریپت‌های ساخت جداول (Migrations)
│   │   │   └── 0001_init.sql
│   │   └── repositories/    # اجرای کوئری‌ها (CRUD)
│   │       ├── mod.rs
│   │       ├── member_repo.rs
│   │       └── plan_repo.rs
│   │
│   ├── services/            # منطق تجاری (Business Logic)
│   │   ├── mod.rs
│   │   ├── member_service.rs # محاسبه تاریخ انقضا، تمدید یک‌کلیکی، فیلتر لحظه‌ای
│   │   └── plan_service.rs
│   │
│   ├── cloud/               # ماژول بکاپ و فضای ابری (Google Drive)
│   │   ├── mod.rs
│   │   ├── oauth.rs         # احراز هویت با مرورگر و توکن محلی
│   │   └── gdrive.rs        # آپلود snapshot فایل sqlite به درایو
│   │
│   └── utils/               # ابزارهای عمومی
│       ├── mod.rs
│       ├── date.rs          # محاسبات تقویم و تبدیل تاریخ‌ها
│       └── errors.rs        # سیستم مدیریت خطاهای سراسری (thiserror/anyhow)
