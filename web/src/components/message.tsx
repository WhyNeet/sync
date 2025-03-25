import { Card } from "@chakra-ui/react";

export function Message({ text }: { text: string }) {
  return <Card.Root w="fit" size="sm" borderRadius="lg">
    <Card.Body >{text}</Card.Body>
  </Card.Root>
}