import React from 'react';
import { StyleSheet, Text, View, ScrollView } from 'react-native';
import styled from 'styled-components';

import { Recipe } from './types';

interface RecipeProps {
  recipe: Recipe;
}

const RecipeView = ({ recipe }: RecipeProps) => {
  return (
    <RecipeContainer>
      <RecipeName> {recipe.name} </RecipeName>
      {recipe.ingredients.map(({ name }, index) => (
        <Ingredient key={index}>{name}</Ingredient>
      ))}
      {recipe.preparation.map(({ description }, index) => (
        <Ingredient key={index}>{description}</Ingredient>
      ))}
    </RecipeContainer>
  );
};

export default RecipeView;

const RecipeContainer = styled(ScrollView)`
  margin: 5px 10px;
`;

const RecipeName = styled(Text)`
  font-size: 32;
`;

const Ingredient = styled(Text)`
  margin: 5px 0;
`;
