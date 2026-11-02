(() => {
  const root = document.documentElement;
  const reduce = window.matchMedia("(prefers-reduced-motion: reduce)").matches;

  const queryTheme = new URLSearchParams(window.location.search).get("theme");
  const stored = localStorage.getItem("sketch-theme");
  const systemDark = window.matchMedia("(prefers-color-scheme: dark)").matches;
  const initial =
    queryTheme === "dark" || queryTheme === "light"
      ? queryTheme
      : stored || (systemDark ? "dark" : "light");
  root.setAttribute("data-theme", initial);

  const themeBtns = document.querySelectorAll("[data-theme-toggle]");
  const syncThemeLabel = () => {
    const next = root.getAttribute("data-theme") === "dark" ? "light" : "dark";
    const label = next === "dark" ? "切换到深色" : "切换到浅色";
    themeBtns.forEach((btn) => btn.setAttribute("aria-label", label));
  };
  syncThemeLabel();

  themeBtns.forEach((btn) => {
    btn.addEventListener("click", () => {
      const next = root.getAttribute("data-theme") === "dark" ? "light" : "dark";
      root.setAttribute("data-theme", next);
      localStorage.setItem("sketch-theme", next);
      syncThemeLabel();
    });
  });

  const toggle = document.querySelector("[data-nav-toggle]");
  const panel = document.querySelector("[data-nav-panel]");
  const setMenu = (open) => {
    toggle?.setAttribute("aria-expanded", String(open));
    toggle?.setAttribute("aria-label", open ? "关闭菜单" : "打开菜单");
    panel?.classList.toggle("is-open", open);
    document.body.style.overflow = open ? "hidden" : "";
  };
  toggle?.addEventListener("click", () => {
    setMenu(toggle.getAttribute("aria-expanded") !== "true");
  });
  if (new URLSearchParams(window.location.search).get("menu") === "1") {
    setMenu(true);
  }
  panel?.querySelectorAll("a").forEach((link) => {
    link.addEventListener("click", () => setMenu(false));
  });
  document.addEventListener("keydown", (event) => {
    if (event.key === "Escape") setMenu(false);
  });

  if (!reduce) {
    const reveals = document.querySelectorAll(".reveal");
    const revealIo = new IntersectionObserver(
      (entries, observer) => {
        entries.forEach((entry) => {
          if (!entry.isIntersecting) return;
          entry.target.classList.add("is-in");
          observer.unobserve(entry.target);
        });
      },
      { threshold: 0.16, rootMargin: "0px 0px -8% 0px" }
    );
    reveals.forEach((el) => revealIo.observe(el));

    const words = document.querySelectorAll(".tagline-word");
    const wordIo = new IntersectionObserver(
      (entries) => {
        entries.forEach((entry) => {
          if (entry.isIntersecting) entry.target.classList.add("is-on");
        });
      },
      { threshold: 0.8, rootMargin: "-28% 0px -42% 0px" }
    );
    words.forEach((word) => wordIo.observe(word));
  } else {
    document.querySelectorAll(".reveal").forEach((el) => el.classList.add("is-in"));
    document.querySelectorAll(".tagline-word").forEach((el) => el.classList.add("is-on"));
  }
})();
