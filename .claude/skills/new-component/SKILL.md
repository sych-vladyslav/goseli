---
name: new-component
description: Scaffold a new React component following the design system. Creates component file with TypeScript types, Tailwind styling, and test stub.
disable-model-invocation: true
argument-hint: [ComponentName storefront|admin]
---

# Scaffold New React Component

Create a new React component following the Goseli design system.

Arguments: $ARGUMENTS[0] = ComponentName, $ARGUMENTS[1] = area (storefront|admin)

## Steps

1. Determine target directory:
   - storefront: `frontend/src/components/storefront/`
   - admin: `frontend/src/components/admin/`
   - shared: `frontend/src/components/shared/`

2. Create component file `ComponentName.tsx`:
   ```tsx
   import { type FC } from 'react';

   interface ComponentNameProps {
     // Define props
   }

   export const ComponentName: FC<ComponentNameProps> = ({ ...props }) => {
     return (
       <div className="...">
         {/* Component content */}
       </div>
     );
   };
   ```

3. Requirements:
   - Use Tailwind classes with design tokens (e.g., `text-primary`, `bg-surface`)
   - Responsive by default (mobile-first)
   - Include aria-* attributes for accessibility
   - Export from the directory's index.ts barrel file

4. Create test stub `ComponentName.test.tsx`:
   ```tsx
   import { render, screen } from '@testing-library/react';
   import { ComponentName } from './ComponentName';

   describe('ComponentName', () => {
     it('renders correctly', () => {
       render(<ComponentName />);
       // TODO: add assertions
     });
   });
   ```
