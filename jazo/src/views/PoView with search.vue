<template>
<v-main fluid>
    <v-layout row wrap>
      <v-flex xs12 class="text-xs-center" mt-5>
        <h1>Points of View</h1>
      </v-flex>
      <v-flex xs12 class="text-xs-center" mt-3>
        <p>Participate in SharingShard with your ideas</p>
      </v-flex>
    </v-layout>

      <v-row>
        Discover the value in this learning experience: {{title}}
      </v-row>

      <v-row  style="height: 500px;">
        <v-col class="pa-8">
                      
          <iframe
              width="100%"
              height="100%"
              :src=url_experiencia
              frameborder="0"
              allow="accelerometer; autoplay; encrypted-media; gyroscope; picture-in-picture"
              allowfullscreen
          ></iframe>
        </v-col>

        <v-col class="pa-4">
        <v-container id="input-usage" >
          <v-row>
            Time: 
            <v-text-field v-model="ss_pov_moment_time">
            Label="Introduce the timemark where you define the moment in the video"
            </v-text-field>
          </v-row>
        </v-container>
          <v-row>
            <v-col
                cols="40"
                md="12"
              >
              <v-textarea
                filled
                v-model="ss_pov_moment"                         

                name="input-7-4"
                label="Moment"
                value=moment
                auto-grow
                rows="15"
                row-height="15"
              ></v-textarea>

            </v-col>
          </v-row>
        </v-col>
      </v-row>
    
      <v-row>
        <v-col class="pa-6">
          <p>URL del video:</p><span>{{url_experiencia}}</span> ---- {{ss_pov_moment}} ---- is here

        </v-col>

        <v-col class="pa-6">  
          <v-row>
            <v-col>
              <v-btn class="pa-2"
                depressed
                color="primary"
                @click=clickea
              >
                Insert
              </v-btn>
            </v-col> 

            <v-col> 
              <v-btn class="pa-2"
                depressed
                color="primary"
              >
                Edit
              </v-btn>
            </v-col>

            <v-col> 
              <v-btn class="pa-2"
                depressed
                color="primary"
              >
                Delete
              </v-btn>
            </v-col> 
          </v-row>
        </v-col>
      </v-row>

    <div>
      <h2>List Points of View</h2>
<div>
    <v-data-table
      :headers="headers"
      :items="author"
      item-key="name"
      class="elevation-1"
      :search="search"
      :custom-filter="filterOnlyCapsText"
    >
      <template v-slot:top>
        <v-text-field
          v-model="search"
          label="Search (UPPER CASE ONLY)"
          class="mx-4"
        ></v-text-field>
      </template>
      <template v-slot:body.append>
        <tr>
          <td></td>
          <td>
            <v-text-field
              v-model="video_id"
              type="number"
              label="Less than"
            ></v-text-field>
          </td>
          <td colspan="4"></td>
        </tr>
      </template>
    </v-data-table>
  </div>



    </div>
      <v-row no-gutters>
          <v-col
            v-for="n in 3"
            :key="n"
            cols="12"
            sm="4"
          >
            <v-card
              class="pl-10 py-5"
              outlined
              tile
            >
              One of three columns
            </v-card>
          </v-col>
      </v-row>
</v-main>
</template>
    
<script>
  export default {
    name: "PoView",
    data() {

      return {
          poview: "",
          title: "Video de pruebas",
          url_experiencia: "https://youtube.com/embed/WCUGI8PGcGw",
          time: 10,
          moment: "cadena de caracteres",
          btn_insert: "Insert",
          btn_edit: "Edit",
          btn_delete: "Delete",
          textarea: null,
          ss_pov_moment: "",
          ss_pov_moment_time: 33,

          search: '',
          author: '',
          PoV:[
            {
              name: 'ss2022_jc.testnet',  //wallet
              video_id: 2,
              comment: "PoV 1 en el video 2 de Jorge",
            },
            {
              name: 'zavala55.testnet',
              video_id: 3,
              comment: "PoV 2 en el video 2 de Jorge",
            }
          ]



      }
    },
    methods: {
        clickea(){
          console.log( "Captura: " + this.ss_pov_moment + "----" + this.ss_pov_moment_time );
        },
        filterOnlyCapsText (value, search, item) {
        return value != null &&
          search != null &&
          typeof value === 'string' &&
          value.toString().toLocaleUpperCase().indexOf(search) !== -1
      },
    },

    computed: {
      headers(){
        return [
        {
          text: 'Points of View',
          align: 'start',
          sortable: false,
          value: 'name',
        },
        { text: 'Owner 1', value: 'owner 1'},
        { text: 'Video', value: '1'},
        { text: 'PoV', value: 'Rollo 1 de PoV'},
        ]
      },
    }
  }
  
</script>
