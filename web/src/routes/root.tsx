import { Container } from '@mui/material';
import { Outlet } from 'react-router';

export const Root = () => {
  return <Container maxWidth="lg" sx={{ height: "100vh" }}>
    <Outlet />
  </Container>
}