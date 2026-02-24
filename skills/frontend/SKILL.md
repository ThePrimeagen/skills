## SolidJS Component Structure

### Rules

1. Every UI component lives in its own file under `src/components/` — one default-exported component per file, named to match the file (e.g. `LeftNav.tsx` exports `LeftNav`).
2. `App.tsx` is strictly a composition root: it imports top-level components, wires up resources/signals, and returns a shallow JSX tree. No markup beyond layout wrappers and `<Show>`/`<Switch>` control flow belongs here.
3. Co-locate a component's CSS in a file alongside it (`ComponentName.css`) and import it from the component file. Shared/global styles stay in `src/App.css`.
4. When a component grows to contain distinct visual sections, extract each section into its own component file under `src/components/` and import it — prefer many small files over one large file.

### Example 1: App as a thin composition root

```tsx
// src/App.tsx
import { createResource, Show } from "solid-js";
import { AuthGate } from "./components/AuthGate";
import { LeftNav } from "./components/LeftNav";
import { MainContent } from "./components/MainContent";
import "./App.css";

const App = () => {
  const [me, { refetch }] = createResource(fetchMe);
  const handleLogout = async () => { await authClient.signOut(); refetch(); };

  return (
    <Show when={!me.loading} fallback={<div class="loading" />}>
      <AuthGate me={me} onLogout={handleLogout}>
        <div class="layout">
          <LeftNav user={me()!.user!} onLogout={handleLogout} />
          <MainContent />
        </div>
      </AuthGate>
    </Show>
  );
};

export default App;
```

### Example 2: A single-responsibility component file

```tsx
// src/components/LeftNav.tsx
import { createSignal, Show } from "solid-js";
import { ProfilePopup } from "./ProfilePopup";
import "./LeftNav.css";

interface LeftNavProps {
  user: { name: string; image: string | null };
  onLogout: () => void;
}

export function LeftNav(props: LeftNavProps) {
  const [showPopup, setShowPopup] = createSignal(false);

  return (
    <nav class="nav">
      <div class="nav-spacer" />
      <div class="profile-area">
        <button class="profile-button" onClick={() => setShowPopup(!showPopup())}>
          <Show when={props.user.image}>
            <img src={props.user.image!} alt={props.user.name} class="profile-avatar" />
          </Show>
          <span class="profile-name">{props.user.name}</span>
        </button>
        <Show when={showPopup()}>
          <ProfilePopup onLogout={props.onLogout} />
        </Show>
      </div>
    </nav>
  );
}
```
