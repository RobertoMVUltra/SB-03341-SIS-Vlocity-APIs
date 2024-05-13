<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>CreateQuote</name>
   <tag></tag>
   <elementGuidId>b191dd0c-4444-4165-b465-27aa3898d9d5</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;Step1\&quot;: {\n        \&quot;CURP\&quot;: \&quot;AABF830222HDFLSR06\&quot;,\n        \&quot;FirstName\&quot;: \&quot;ENRIQUE\&quot;,\n        \&quot;LastName\&quot;: \&quot;ALVAREZ BUSTAMANTE\&quot;,\n        \&quot;RFC\&quot;: \&quot;AABF8302221S7\&quot;,\n        \&quot;PhoneNumber\&quot;: \&quot;8127320159\&quot;,\n        \&quot;Email\&quot;: \&quot;davidnovelo@gmail.com\&quot;,\n        \&quot;BirthDate\&quot;: \&quot;1983-02-22\&quot;\n    },\n    \&quot;Step2\&quot;: {\n        \&quot;Address\&quot;: {\n            \&quot;Street\&quot;: \&quot;kentucky\&quot;,\n            \&quot;ExternalNumber\&quot;: \&quot;425\&quot;,\n            \&quot;InternalNumber\&quot;: \&quot;\&quot;,\n            \&quot;City\&quot;: \&quot;Monterrey\&quot;,\n            \&quot;Country\&quot;: \&quot;México\&quot;,\n            \&quot;State\&quot;: \&quot;Nuevo León\&quot;,\n            \&quot;Town\&quot;: \&quot;Kennedy\&quot;,\n            \&quot;PostalCode\&quot;: \&quot;64260\&quot;\n        }\n    },\n    \&quot;Step3\&quot;: {\n        \&quot;BenificiaryInformation\&quot;: [\n            {\n                \&quot;BirthDate\&quot;: \&quot;1986-10-15\&quot;,\n                \&quot;FirstName\&quot;: \&quot;BENJAMIN\&quot;,\n                \&quot;email\&quot;: \&quot;\&quot;,\n                \&quot;KinshipExternalCode\&quot;: \&quot;0005\&quot;,\n                \&quot;LastName\&quot;: \&quot;NOVELO\&quot;,\n                \&quot;Percentage\&quot;: 100,\n                \&quot;NumBen\&quot;: 1,\n                \&quot;Address\&quot;: {\n                    \&quot;Street\&quot;: \&quot;kentucky\&quot;,\n                    \&quot;ExternalNumber\&quot;: \&quot;425\&quot;,\n                    \&quot;InternalNumber\&quot;: \&quot;\&quot;,\n                    \&quot;City\&quot;: \&quot;Kennedy\&quot;,\n                    \&quot;Country\&quot;: \&quot;México\&quot;,\n                    \&quot;State\&quot;: \&quot;Nuevo León\&quot;,\n                    \&quot;Town\&quot;: \&quot;Monterrey\&quot;,\n                    \&quot;PostalCode\&quot;: \&quot;64260\&quot;\n                }\n            }\n        ]\n    },\n    \&quot;Step4\&quot;: {\n        \&quot;Package\&quot;: {\n            \&quot;branch\&quot;: \&quot;VI47\&quot;,\n            \&quot;Code\&quot;: \&quot;SEGURO_VI_APF1_ROOT\&quot;,\n            \&quot;StartDate\&quot;: \&quot;2023-10-31\&quot;,\n            \&quot;EndDate\&quot;: \&quot;2024-10-31\&quot;,\n            \&quot;Amount\&quot;: 8696.6,\n            \&quot;total\&quot;: 8696.6,\n            \&quot;taxAmount\&quot;: 0,\n            \&quot;priceWithExpense\&quot;: 8696.6,\n            \&quot;expense\&quot;: 0,\n            \&quot;numInstallments\&quot;: \&quot;001\&quot;,\n            \&quot;expenseOnFirstInstallment\&quot;: false,\n            \&quot;installmentAmount\&quot;: 8696.6\n        },\n        \&quot;PackageCoverage\&quot;: [\n            {\n                \&quot;code\&quot;: \&quot;VI10\&quot;,\n                \&quot;price\&quot;: 250,\n                \&quot;insuredAmount\&quot;: 500000,\n                \&quot;name\&quot;: \&quot;Muerte Accidental\&quot;\n            },\n            {\n                \&quot;code\&quot;: \&quot;VI58\&quot;,\n                \&quot;price\&quot;: 433,\n                \&quot;insuredAmount\&quot;: 50000,\n                \&quot;name\&quot;: \&quot;Indemnización Hospitalaria por Accidente\&quot;\n            },\n            {\n                \&quot;code\&quot;: \&quot;VI67\&quot;,\n                \&quot;price\&quot;: 1370.6,\n                \&quot;insuredAmount\&quot;: 2500,\n                \&quot;name\&quot;: \&quot;Indemnización Diaria por Hospitalización\&quot;\n            },\n            {\n                \&quot;code\&quot;: \&quot;VI33\&quot;,\n                \&quot;price\&quot;: 1925,\n                \&quot;insuredAmount\&quot;: 500000,\n                \&quot;name\&quot;: \&quot;Incapacidad Total o Incapacidad Permanente Total o Invalidez\&quot;\n            },\n            {\n                \&quot;code\&quot;: \&quot;VI00\&quot;,\n                \&quot;price\&quot;: 4065,\n                \&quot;insuredAmount\&quot;: 500000,\n                \&quot;name\&quot;: \&quot;Fallecimiento\&quot;\n            },\n            {\n                \&quot;code\&quot;: \&quot;VI95\&quot;,\n                \&quot;price\&quot;: 653,\n                \&quot;insuredAmount\&quot;: 100000,\n                \&quot;name\&quot;: \&quot;Enfermedades Graves\&quot;\n            }\n        ]\n    },\n    \&quot;Step5\&quot;: {\n        \&quot;Bank\&quot;: \&quot;072\&quot;,\n        \&quot;CardCVV\&quot;: \&quot;123\&quot;,\n        \&quot;FirstName\&quot;: \&quot;david\&quot;,\n        \&quot;LastName\&quot;: \&quot;novelo carrillo\&quot;,\n        \&quot;PaymentPeriod\&quot;: \&quot;001\&quot;,\n        \&quot;TypeOfCard\&quot;: \&quot;04\&quot;,\n        \&quot;Amount\&quot;: 8696.6,\n        \&quot;IsRecurringPayment\&quot;: true,\n        \&quot;CardMonth\&quot;: 1,\n        \&quot;CardNumber\&quot;: 1234123412341234,\n        \&quot;CardYear\&quot;: 2030\n    }\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>8.6.9</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://segbanortesfi--qa.sandbox.my.salesforce.com/services/apexrest/vlocity_ins/v1/integrationprocedure/ClbIns_CreateQuote/</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
