import '@fontsource-variable/dm-sans';
import { Outlet } from 'react-router';
import { Container } from '@chakra-ui/react';

export const Root = () => {
  return <Container h="100vh" w="100vw" bg="white">
    <Outlet />
  </Container>
}