import faker from '@faker-js/faker/locale/en';

export const postTags: Model.PostTagR[] = [
  { id: faker.datatype.uuid(), name: 'Vue.JS', icon: 'mdi:vuejs' },
  { id: faker.datatype.uuid(), name: 'Rust', icon: 'mdi:language-rust' },
  { id: faker.datatype.uuid(), name: 'Figma', icon: 'feather:figma' },
  { id: faker.datatype.uuid(), name: 'Linux', icon: 'cib:linux' },
  { id: faker.datatype.uuid(), name: 'Angular', icon: 'mdi:angularjs' },
  { id: faker.datatype.uuid(), name: 'React.JS', icon: 'mdi:react' },
  { id: faker.datatype.uuid(), name: 'SQL', icon: 'fluent:database-24-regular' },
  { id: faker.datatype.uuid(), name: 'Go', icon: 'mdi:language-go' },
  { id: faker.datatype.uuid(), name: 'UI/UX Design', icon: 'fluent:design-ideas-24-regular' },
  { id: faker.datatype.uuid(), name: 'Others' },
];
