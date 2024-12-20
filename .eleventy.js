import { IdAttributePlugin, InputPathToUrlTransformPlugin, HtmlBasePlugin } from "@11ty/eleventy";
import { feedPlugin } from "@11ty/eleventy-plugin-rss";
import pluginSyntaxHighlight from "@11ty/eleventy-plugin-syntaxhighlight";
import pluginNavigation from "@11ty/eleventy-navigation";
import { eleventyImageTransformPlugin } from "@11ty/eleventy-img";

import pluginFilters from "./_config/filters.js";

export default async function (eleventyConfig) {
  // Add a custom collection for Jupyter notebooks
  eleventyConfig.addCollection('aoc2020', function (collection) {
    return collection.getFilteredByGlob('2020/*.md')
      .map(item => {
        const dateMatch = item.fileSlug.match(/^(\d{4})-(\d{1,2})-(\d{1,2})/);
        if (dateMatch) {
          const [_, year, month, day] = dateMatch;
          item.date = new Date(year, month - 1, day, 12, 0, 0);
        }
        return item;
      });
  });
  eleventyConfig.addCollection('aoc2021', function (collection) {
    return collection.getFilteredByGlob('2021/*.md')
      .map(item => {
        const dateMatch = item.fileSlug.match(/^(\d{4})-(\d{1,2})-(\d{1,2})/);
        if (dateMatch) {
          const [_, year, month, day] = dateMatch;
          item.date = new Date(year, month - 1, day, 12, 0, 0);
        }
        return item;
      });
  });
  eleventyConfig.addCollection('aoc2023', function (collection) {
    return collection.getFilteredByGlob('2023/*.md')
      .map(item => {
        const dateMatch = item.fileSlug.match(/^(\d{4})-(\d{1,2})-(\d{1,2})/);
        if (dateMatch) {
          const [_, year, month, day] = dateMatch;
          item.date = new Date(year, month - 1, day, 12, 0, 0);
        }
        return item;
      });
  });

  eleventyConfig.addBundle("css");
  eleventyConfig.addPassthroughCopy("dist/style.css");
  eleventyConfig.addBundle("js");

  eleventyConfig.addPlugin(pluginSyntaxHighlight, {
    preAttributes: { tabindex: 0 }
  });
  eleventyConfig.addPlugin(pluginNavigation);
  eleventyConfig.addPlugin(HtmlBasePlugin);
  eleventyConfig.addPlugin(InputPathToUrlTransformPlugin);
  // Filters
  eleventyConfig.addPlugin(pluginFilters);

  eleventyConfig.addPlugin(IdAttributePlugin, {
    // by default we use Eleventy’s built-in `slugify` filter:
    // slugify: eleventyConfig.getFilter("slugify"),
    // selector: "h1,h2,h3,h4,h5,h6", // default
  });

  eleventyConfig.addShortcode("currentBuildDate", () => {
    return (new Date()).toISOString();
  });


};

export const config = {
  pathPrefix: '/adventofcode/',
  dir: {
    output: 'dist',
    input: '.',
    includes: '_includes',
    layouts: '_includes/layouts',
    data: '_globals',
  }
}