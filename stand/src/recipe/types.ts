import Pbf from 'pbf';
import { Recipe as RecipeProto } from './_auto';

export interface Recipe {
  name: string;
  ingredients: {
    name: string;
  }[];
  preparation: {
    description: string;
  }[];
}

export async function fetchRecipe(): Promise<Recipe> {
  const req = new XMLHttpRequest();
  return new Promise((resolve, reject) => {
    // resolve({
    //   name: 'Potrawka z bigosu z kiełbasą czosnkową',
    //   ingredients: [{ name: 'bigos' }, { name: 'kiełbasa' }],
    //   preparation: [{ description: 'zrób zakupy' }],
    // });
    req.open('GET', 'https://recipes-git-master.ptm.now.sh', true);
    req.responseType = 'arraybuffer';
    req.onerror = reject;
    req.onload = () => {
      const arrayBuffer = req.response;
      if (arrayBuffer) {
        const byteArray = new Uint8Array(arrayBuffer);
        const pbf = new Pbf(byteArray);
        const recipe = RecipeProto.read(pbf) as Recipe;
        resolve(recipe);
      }
    };
    req.send(null);
  });
}
