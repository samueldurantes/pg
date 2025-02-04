import React from 'react';
import ReactDOM from 'react-dom/client';
import { createBrowserRouter, RouterProvider } from 'react-router-dom';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { Toaster } from './components/ui/toaster.tsx';

import AuthLayout from './components/auth/AuthLayout.tsx';
import Home from './components/home/Home.tsx';
import Login from './components/auth/Login';
import Register from './components/auth/Register';
import Game from './components/game/Game.tsx';

import './index.css';
import Join from './components/game/Join.tsx';

const queryClient = new QueryClient();

const router = createBrowserRouter([
  {
    path: '/login',
    element: <Login />,
  },
  {
    path: '/register',
    element: <Register />,
  },
  {
    element: <AuthLayout />,
    children: [
      {
        path: '/',
        element: <Home />,
      },
      {
        path: '/join/:id',
        element: <Join />,
      },
      {
        path: '/game/:id',
        element: <Game />,
      },
    ],
  },
]);

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <QueryClientProvider client={queryClient}>
      <React.Suspense>
        <RouterProvider router={router} />
        <Toaster />
      </React.Suspense>
    </QueryClientProvider>
  </React.StrictMode>
);
