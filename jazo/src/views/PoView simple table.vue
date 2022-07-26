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
                label="Create a Point of View about this challenge"
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
                @click=clickea_insert_moment()
              >
                Insert
              </v-btn>
            </v-col> 

            <v-col> 
              <v-btn class="pa-2"
                depressed
                color="primary"
                @click=clickea_edit_moment()

              >
                Edit
              </v-btn>
            </v-col>

            <v-col> 
              <v-btn class="pa-2"
                depressed
                color="primary"
                @click=clickea_delete_moment()
              >
                Delete
              </v-btn>
            </v-col> 
          </v-row>
        </v-col>
      </v-row>


    <div v-if=display_PoV>
      <h2>List Points of View</h2>
      <div @click="dialog = true">
        <v-data-table
          :headers="headers"
          :items="PoV"
          item-key="name"
          :items-per-page="15"
          class="elevation-1"


        ></v-data-table>
      </div>
    </div>



  <div class="text-center">
    <v-dialog
      v-model="dialog"
      width="500"
    >
      <template v-slot:activator="{ on, attrs }">
        <v-btn
          color="red lighten-2"
          dark
          v-bind="attrs"
          v-on="on"
        >
          Click Me
        </v-btn>
      </template>

      <v-card>
        <v-card-title class="text-h5 grey lighten-2">
          Rewards Assigment
        </v-card-title>

        <v-card-text>
          <p>Available Reward: XX</p>
          <p>The award for this participation is:</p>
          <p><v-text-field v-model="ss_pov_moment_award" reverse></v-text-field></p>
        </v-card-text>

        <v-divider></v-divider>

        <v-card-actions>
          <v-spacer></v-spacer>
          <v-btn
            color="primary"
            dark
            @click="dialog = false"
          >
            Assign
          </v-btn>
          <v-btn
            color="primary"
            dark
            @click="dialog = false"
          >
            Cancel
          </v-btn>

        </v-card-actions>
      </v-card>
    </v-dialog>
  </div>





</v-main>
</template>
    
<script>
import * as nearAPI from 'near-api-js'
const { connect, WalletConnection, keyStores, Contract } = nearAPI;

const CONTRACT_ID = "dev-1658426475128-11579451230587";
const config = {
  networkId: 'testnet',
  keyStore: new keyStores.BrowserLocalStorageKeyStore(),
  nodeUrl: 'https://rpc.testnet.near.org',
  walletUrl: 'https://wallet.testnet.near.org',
  helperUrl: 'https://helper.testnet.near.org',
  explorerUrl: 'https://explorer.testnet.near.org'
};


  export default {
    name: "PoView",

  created(){
    // this.disp_pov();
    /*
    console.log("POV created: " + window.location.href);
    console.log(this.$route.params);
    const cual = this.$route.params.video_id;
    console.log( cual );
    console.log(this.$route.params.video_id);
    */
    this.display_PoV = false;
    this.disp_experiences_for_pov(this.$route.params.video_id);
    this.display_PoV = true;

  },  


    data() {

    return {
        exp_info: "",

        dialog: false,


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
        ss_pov_moment_award: 0,

        headers: [
          {
            text: 'Points of View',
            align: 'start',
            sortable: false,
            value: 'name',
          },
          { text: 'Owner 1', value: 'owner'},
          { text: 'Video', value: 'PoV'},
        ],

        /*
          {
            name: 'Video 1',
            owner: 'ss2022_jc.testnet',  //wallet
            video_id: 2,
            PoV: "PoV 1 en el video 2 de Jorge",
          }
          */
        PoV:[ ],
      }
    },
    methods: {
      clickea_insert_moment(){
        console.log( "clickea_insert_moment -- Captura: " + this.ss_pov_moment);
        this.insert_pov( 1 );
      },
      clickea_edit_moment(){
        console.log( "clickea_edit_moment -- Captura: " + this.ss_pov_moment);
      },
      clickea_delete_moment(){
        console.log( "clickea_delete_moment -- Captura: " + this.ss_pov_moment );
      },

      async insert_pov( video_id ){
          const near = await connect(config);
          const wallet = new WalletConnection(near, 'SharingShard');

          const contract = new Contract(wallet.account(), CONTRACT_ID, {
              viewMethods: ['get_experience'],
              changeMethods:  ['set_pov'],
              sender: wallet.account()
          });

        console.log( "video id; " + video_id);
        this.exp_info = await contract.get_experience({
          video_n: 2 // video_id
        });

          /*
          {
          name: 'Video 2',
          owner: 'zavala55.testnet',
          PoV: "PoV 2 en el video 2 de Jorge",
          }
          */    

          console.log( "Experience: " + this.exp_info);
          console.log( this.exp_info.pov.length );
          console.log( wallet.getAccountId());

          if ( this.exp_info.pov.length == 0 ){  // No hay ningun comentario
            console.log( "insertando ...... ")
            this.exp_info = await contract.set_pov({
                video_n: 2, //video_id,
                pov: this.ss_pov_moment
            });
            console.log("Comentario insertado");
          }
      },

      async disp_experiences_for_pov( video_id ){
        const near = await connect(config);
        const wallet = new WalletConnection(near, 'SharingShard');

        const contract = new Contract(wallet.account(), CONTRACT_ID, {
          viewMethods:  ['get_number_of_experiences', 'get_user_exp','get_experience'],
          sender: wallet.account()
        });

        this.exp_info = await contract.get_experience({
          video_n: video_id
        });


/*
          console.log( "disp_experiences_for_pov -- parte 2" );

          console.log( this.exp_info ); 

          console.log( this.exp_info.pov )
          console.log( this.exp_info.pov.length )
          console.log( this.exp_info.pov[0] )
          console.log( this.exp_info.pov[0].length )
          console.log( this.exp_info.pov[0][0] )
          console.log( this.exp_info.pov[0][1] )
*/
          /*
          {
            name: 'Video 2',
            owner: 'zavala55.testnet',
            video_id: 3,
            PoV: "PoV 2 en el video 2 de Jorge",
          }
          */

         let PoVitem = {};
         const PoV_listing = [];
         //this.PoV.pop();
          for ( let i = 0 ; i < this.exp_info.pov.length ; i++  ){
            PoVitem = {};
            PoVitem.name = "Point of View: " + i;
            PoVitem.owner = this.exp_info.pov[i][0];
            PoVitem.video_id = i;
            PoVitem.PoV = this.exp_info.pov[i][1];

            PoV_listing.push( PoVitem);
          }

         this.PoV = PoV_listing;
        }

          

      },
/*      filterOnlyCapsText (value, search, item) {
      return value != null &&
        search != null &&
        typeof value === 'string' &&
        value.toString().toLocaleUpperCase().indexOf(search) !== -1
      },


        To use in reading the information for the experience

          "url" : this.experienciesview[video_id - 1].url,
          "title" : this.experienciesview[video_id - 1].title,
          "description" : this.experienciesview[video_id - 1].description,
          "id" : this.experienciesview[video_id - 1].id,
          "owner" : this.experienciesview[video_id - 1].owner,
          "reward" : this.experienciesview[video_id - 1].reward,

          "time" : this.experienciesview[video_id - 1].time,
          "moment" : this.experienciesview[video_id - 1].moment,


*/
  }


</script>
