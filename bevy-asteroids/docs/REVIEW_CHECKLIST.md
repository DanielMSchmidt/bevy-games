# Merge Review Checklist

The AI must evaluate the following:

## 1. ECS Architecture

- Are components data-only?
- Is logic placed in systems?
- Is input separated from physics?
- Are responsibilities cleanly separated?

---

## 2. Scheduling

- Are systems placed in appropriate schedules?
- Is ordering explicit when required?
- Is delta time used correctly?

---

## 3. Performance

- Avoid allocations in hot systems
- Avoid repeated expensive queries
- Avoid cloning unnecessarily

---

## 4. Code Clarity

- Naming clear?
- Modules well scoped?
- Minimal but readable?

---

## 5. Gameplay Integrity

- True inertia preserved?
- Motion consistent?
- No hidden frame dependence?

---

## Decision Output Format

## Decision: Merge / Don’t Merge

Reasons:
- ...

If Don’t Merge:
- Provide smallest fix set
- Do NOT rewrite entire system
