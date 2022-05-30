import * as React from "react";
import { Button, Container, TextField } from "@mui/material";
import { useForm } from "react-hook-form";
import { object, string, number, date } from "yup";

let userSchema = object({
  name: string().required(),
  age: number().required().positive().integer(),
  address: string().required(),
  createdOn: date().default(() => new Date()),
});

export default function Form() {
  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm();
  const onSubmit = async (data) => {
    // parse and assert validity
    await userSchema.validate(data);
    console.log(data);
  };

  return (
    <Container maxWidth="lg" style={{ padding: "100px" }}>
      <form onSubmit={handleSubmit(onSubmit)}>
        <TextField
          id="name"
          label="Name"
          variant="outlined"
          required
          {...register("name")}
        />
        <TextField
          id="age"
          label="Age"
          variant="outlined"
          type="number"
          required
          {...register("age")}
        />
        <TextField
          id="address"
          label="Address"
          variant="outlined"
          required
          {...register("address")}
        />
        <Button type="submit">Submit</Button>
      </form>
    </Container>
  );
}
